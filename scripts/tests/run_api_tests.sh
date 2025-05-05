#!/usr/bin/env bash

set -euo pipefail

# ------------------------------------------------------------------------------
# Arguments
# ------------------------------------------------------------------------------

verbose=0

params="$(getopt -o v -- "${@}")"
eval set -- "${params}"

while [ "$#" -gt 0 ]
do
    case "${1}" in
    -v)
        verbose=$((verbose + 1))
        shift
        ;;
    --)
        shift
        break
        ;;
     *)
        shift
        ;;
    esac
done

db_name="${1:-axum_test}"
mode="${2:-debug}"

# ------------------------------------------------------------------------------
# Variables
# ------------------------------------------------------------------------------

export DATABASE_URL="postgres://demo:demo@localhost:5432/${db_name}"
export PGPASSWORD=demo

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
PURPLE='\033[1;35m'
YELLOW='\033[1;33m'
BLUE='\033[1;34m'
NC='\033[0m' # No Color

FAILED_TESTS=()
PASSED_TESTS=()
HURL_LOG_DIR="./.hurl/logs"
SERVER_LOG_FILE="./.hurl/axum-skeleton.log"
HURL_ENV=./data/tests/vars.env

mkdir -p "${HURL_LOG_DIR}"
rm -f "${HURL_LOG_DIR}"/*.log "${SERVER_LOG_FILE}"

# ------------------------------------------------------------------------------
# Cleanup function
# ------------------------------------------------------------------------------

function cleanup {
    echo -e "\n${PURPLE}Stopping axum-skeleton (PID ${SERVER_PID})...${NC}"

    kill ${SERVER_PID} || true
    wait ${SERVER_PID} || true

    echo -e "${GREEN}Finished${NC}"
}

trap cleanup EXIT
trap cleanup SIGINT SIGTERM SIGQUIT SIGHUP SIGKILL ERR

# ------------------------------------------------------------------------------
# Start the server
# ------------------------------------------------------------------------------

echo -ne "${PURPLE}Starting axum-skeleton...${NC}"

# If in release mode, expect the binary to be already built (CI mode)
if [[ "${mode}" == "debug" ]]
then
    cargo build -p axum-skeleton 1>/dev/null 2>&1
elif [[ "${mode}" == "release" ]]
then
    if [[ ! -e ./target/release/axum-skeleton ]]
    then
        echo "Error: axum-skeleton binary not found."
        exit 1
    fi
else
    echo "Error: Invalid mode. Use 'debug' or 'release'."
    exit 1
fi

./target/${mode}/axum-skeleton > "${SERVER_LOG_FILE}" 2>&1 &
SERVER_PID=$!

echo -e "${PURPLE} done (PID ${SERVER_PID})${NC}"

# ------------------------------------------------------------------------------
# Populate the database with test data
# ------------------------------------------------------------------------------

echo -ne "${PURPLE}Seeding database...${NC}"

psql \
    -h 127.0.0.1 \
    -d ${db_name} \
    -U demo \
    -p 5432 \
    -q \
    -f ./data/tests/seed.sql

echo -e "${PURPLE} done${NC}"

# ------------------------------------------------------------------------------
# Run tests
# ------------------------------------------------------------------------------

# Wait for the server to be ready
echo -ne "${PURPLE}Waiting server to be ready...${NC}"
sleep 2
echo -e "${PURPLE} done${NC}"

echo -e "${PURPLE}Running tests...${NC}"

hurl_verbose=""
if [[ ${verbose} -eq 1 ]]
then
    hurl_verbose="--verbose"
elif [[ ${verbose} -eq 2 ]]
then
    hurl_verbose="--very-verbose"
fi

for hurl_file in $(find . -name "*.hurl")
do
    if [[ ! -f "${hurl_file}" ]]
    then
        continue
    fi

    test_name=$(basename "${hurl_file}")
    log_file="${HURL_LOG_DIR}/${test_name}.log"

    echo -e "${YELLOW}→ Running ${test_name}...${NC}"

    if hurl ${hurl_verbose} --variables-file "${HURL_ENV}" "${hurl_file}" >"${log_file}" 2>&1
    then
        echo -e "  ${GREEN}✔ Passed${NC}"
        PASSED_TESTS+=("$hurl_file")
    else
        echo -e "  ${RED}✘ Failed${NC}"
        echo -e "${BLUE}---- Logs for ${test_name} ----${NC}"
        cat "${log_file}"
        echo -e "${HURL_ENV}---- axum-skeleton logs ----${NC}"
        tail -n 40 "${SERVER_LOG_FILE}"
        FAILED_TESTS+=("${hurl_file}")
    fi
done

# ------------------------------------------------------------------------------
# Summary
# ------------------------------------------------------------------------------

echo -e "\n${PURPLE}Test Summary:${NC}"

echo -e "${GREEN}✔ Passed: ${#PASSED_TESTS[@]}${NC}"
for t in "${PASSED_TESTS[@]}"
do
    echo -e "  ${t}";
done

if [ "${#FAILED_TESTS[@]}" -gt 0 ]
then
    echo -e "\n${RED}✘ Failed: ${#FAILED_TESTS[@]}${NC}"
    for t in "${FAILED_TESTS[@]}"; do
        echo -e " ${t}"
        echo -e "   ➤ Rerun: hurl ${hurl_verbose} --variables-file "${HURL_ENV}" ${t}"
    done
    exit 1
else
    echo -e "\n${GREEN}🎉 All tests passed!${NC}"
fi
