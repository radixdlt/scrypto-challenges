const eventsPerUser: Map<string, Array<any>> = new Map();

export async function GET({params}: any) {
    const user = params["user"];
    let events = eventsPerUser.get(user);
    if (events == null) {
        events = [];
    }
    return new Response(JSON.stringify([...events].reverse()), {status: 200});
}

export async function POST({request, params}: any) {
    const user = params["user"];
    const event = await request.json();
    const events = eventsPerUser.get(user);
    if (events == null) {
        eventsPerUser.set(user, [event]);
    } else {
        events.push(event);
    }
    return new Response(null, {status: 204});
}

export function DELETE() {
    eventsPerUser.clear();
    return new Response(null, {status: 204});
}