EXIT=0

for test in $(cd test; ls); do
    printf "%-40s" "${test}..."
    if cmp --silent <(test/$test) .golden/$test; then
        echo 'PASSED!'
    else
        echo 'FAILED!'
        EXIT=1
    fi
done

exit $EXIT
