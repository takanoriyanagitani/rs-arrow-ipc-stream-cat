#!/bin/sh

istream="./sample.d/input.stream.ipc"

gencsv(){
	echo timestamp,severity,body
	echo 2025-10-16T00:32:24.012345Z,INFO,apt update done
	echo 2025-10-16T00:32:24.012345Z,WARN,apt update failure
}

geninput(){
	echo generating input ipc stream file...

	which csv2arrow2ipc | fgrep -q csv2arrow2ipc || exec sh -c '
		echo csv2arrow2ipc missing.
		echo see github.com/takanoriyanagitani/go-csv2arrow2ipc to install it.
		exit 1
	'

	mkdir -p ./sample.d

	gencsv | csv2arrow2ipc -comma=, > "${istream}"
}

test -f "${istream}" || geninput

which arrow-file-to-stream | fgrep -q arrow-file-to-stream || exec sh -c '
	echo arrow-file-to-stream missing.
	exit 1
'

arrow-file-to-stream "${istream}" |
	./rs-arrow-ipc-stream-cat \
		--fields 0,2
