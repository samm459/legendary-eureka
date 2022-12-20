for await (const socket of Network.bind(3000)) {
    for await (const message of socket) {
        socket.write("You said: ", message)
    }
}