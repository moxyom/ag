export interface VoteAdmin {
    title: string,
    answers: {
        message: string
        votes: number
    }[]
}

export interface VoteClient {
    title: string,
    answers: string[]
    id: number
}

