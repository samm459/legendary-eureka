for await (const line of Process.stdin) {
    print("You said: ", line)
}