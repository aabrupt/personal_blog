#!/bin/bash


pushd client && pnpm run dev &
pushd server && cargo watch -x "run"
&& fg
