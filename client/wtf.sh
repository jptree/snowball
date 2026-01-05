cargo build --target x86_64-pc-windows-gnu -j 4 &&
cp target/x86_64-pc-windows-gnu/debug/snowball.exe . &&
exec ./snowball.exe "$@"