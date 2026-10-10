import { ref } from "vue"

type WSStatus = 'CONNECTING' | 'OPEN' | 'CLOSING' | 'CLOSED'

export const wsConnectTo = (
    url: string,
    onMessage: (data: any) => void
) => {
    let socket: WebSocket | null = null
    let reconnectTimer: ReturnType<typeof setTimeout> | null = null
    let retries = 0
    let manuallyClosed = false

    const status = ref<WSStatus>('CONNECTING')

    const connect = () => {
        if (manuallyClosed) return

        status.value = 'CONNECTING'
        socket = new WebSocket(url)

        socket.onopen = () => {
            retries = 0
            status.value = 'OPEN'
            console.log('[WS] Connected')
        }

        socket.onmessage = (event: MessageEvent) => {
            try {
                onMessage(JSON.parse(event.data))
            } catch (error) {
                console.error('[WS] Invalid JSON message', error)
            }
        }

        socket.onerror = (event) => {
            console.error('[WS] Error', event)
            // onclose déclenche normalement la reconnexion.
        }

        socket.onclose = () => {
            status.value = 'CLOSED'
            socket = null

            if (manuallyClosed) return

            const delay = retries > 5 ? 2000 : 1000
            retries++

            reconnectTimer = setTimeout(connect, delay)
        }
    }

    const send = (data: any) => {
        if (socket?.readyState !== WebSocket.OPEN) {
            console.warn('[WS] Cannot send: socket is not open')
            return
        }

        socket.send(JSON.stringify(data))
    }

    const close = () => {
        manuallyClosed = true

        if (reconnectTimer !== null) {
            clearTimeout(reconnectTimer)
            reconnectTimer = null
        }

        socket?.close()
        socket = null
        status.value = 'CLOSED'
    }

    connect()

    return {
        status,
        send,
        close,
    }
}
