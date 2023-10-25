#!/usr/bin/env bash
checksum() {
    echo $(sha256sum $@ | awk '{print $1}')
}

change_log_file="./CHANGELOG.md"
version="## $@"
version_prefix="## [0-9]{1,2}\."
start=0
CHANGE_LOG=""
while read line; do
    if [[ $line == *"$version"* ]]; then
        start=1
        continue
    fi
    if [[ $line =~ $version_prefix ]] && [ $start == 1 ]; then
        break;
    fi
    if [ $start == 1 ]; then
        CHANGE_LOG+="$line\n"
    fi
done < ${change_log_file}

OUTPUT=$(cat <<-END
## Changelog\n
${CHANGE_LOG}\n
END
)

echo -e ${OUTPUT}
