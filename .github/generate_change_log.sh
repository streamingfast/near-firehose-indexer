#!/usr/bin/env bash

change_log_file="./CHANGELOG.md"
section_prefix="^## "

start=0
CHANGE_LOG=""
while read line; do
    if [[ $line =~ $section_prefix ]] && [ $start == 0 ]; then
        start=1
        continue
    fi

    if [[ $line =~ $section_prefix ]] && [ $start == 1 ]; then
        break;
    fi

    if [ $start == 1 ]; then
        CHANGE_LOG+="$line\n"
    fi
done < ${change_log_file}

OUTPUT=$(cat <<-END
## Changelog\n
${CHANGE_LOG}
END
)

echo -e ${OUTPUT}
