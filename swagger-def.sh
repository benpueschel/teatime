#!/usr/bin/env bash
# vi: ft=bash

name="$1"

if [ -z "$name" ]; then
  echo "Usage: $0 <name>"
  exit 1
fi

rusty_type() {
  case "$1" in
	string) echo "String" ;;
	integer) echo "i64" ;;
	boolean) echo "bool" ;;
	*) echo "$1" ;;
  esac
}

props="$(jq -r ".definitions.$name.properties" swagger.json)"
names="$(echo "$props" | jq -r 'keys[]')"

for name in $names; do
	desc="$(echo "$props" | jq -r ".$name.description")"
	type="$(echo "$props" | jq -r ".$name.type")"
	type="$(rusty_type "$type")"

	# if desc is not empty and not null
	if [ -n "$desc" ] && [ "$desc" != "null" ]; then
		# put '///' before each line in desc
		desc="$(echo "$desc" | sed 's/^/\/\/\/ /')"
		echo "$desc"
	fi
	echo "$name: Option<$type>,"
done
