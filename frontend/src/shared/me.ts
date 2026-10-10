export const getMeInfo = async () => {
        try {
        // will create a token corresponding to the room
        const ress = await fetch(`/me`).then(a => a.json())
        return ress as { roomId: number, admin: boolean }
    } catch (e) {
        return null
    }
}


export const ereaseCookie = () => {
    document.cookie = "auth=; Max-Age=-1;"
}
