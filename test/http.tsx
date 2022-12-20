interface Application {
    posts: Post[],
    users: User[],
}

const { get, body } = Http.listen(3000);
const { define, push } = Container.init<Application>("data/container.js")

/**
 * 
 */
@define("posts")
class Post {
    title: string
    body: JSX.Element

    get("/")
    async index(@body ) {
        await push()
    }
}

/**
 * 
 */
@define("users")
class User {
    title: string
    body: JSX.Element

    @get("/")
    async index() {
        container.push()
    }
}

/**
 * 
 */
server.get("/", async (req, res) => {
    res.status(201).send(
        <h1></h1>
    )
});

for (const [req, res] of server.get("/")) {
    
}

/**
 * 
 * 
 */
for await (const [req, res] of server.get("/api")) {
    res.status(201).send([{ text: "Hello World!" }])
}

/**
 * 
 * 
 */
for await (const [req, res] of server.post<object>("/api")) {
    res.status(201).send({ text: "Hello World!" })
}