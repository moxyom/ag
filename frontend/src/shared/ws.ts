import { useWebSocket } from "@vueuse/core"

export const wsConnectTo = (url: string, onMessage: (data: any) => void) => {
    const { status, send } = useWebSocket(url, {
        autoReconnect: {
            retries: Infinity,
            delay: retries => retries > 5 ? 2000 : 1000,
        },
        onMessage: (_, event) => onMessage(JSON.parse(event.data)),
    })

    return {
        status,
        send: (data: any) => send(JSON.stringify(data))
    }
}
