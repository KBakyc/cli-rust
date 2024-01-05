#!/usr/bin/env bash

echo "Removing all \"target\" directories ..."
fd -I '^target$' -td -X rm -r
echo "Done."
