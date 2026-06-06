#!/bin/bash

tree -L 2

echo "lib.rs:"
cat lib.rs
echo

echo "core.rs:"
cat core.rs
echo

echo "core/runtime.rs"
cat core/runtime.rs
echo


echo "modelview.rs:"
cat modelview.rs
echo

echo "modelview/runtime.rs:"
cat modelview/runtime.rs
echo

