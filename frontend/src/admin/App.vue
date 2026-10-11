<template>
    <Layout>
        <Header :code="state.scene == 'room-created' ? state.roomId : null">
            <div v-if="state.scene == 'room-created'">
                <InscriptionToggle :joinable="state.joinable" @on-change="onRoomOpenChange" />
                <p class="participant-count"><small>{{ state.participantsCount }} participants</small></p>
            </div>
        </Header>

        <div v-if="state.scene == 'room-creation'" class="room-creation-scene">
            <button @click="onCreateRoomBtnClick">Créer une séance</button>
            <Notice :message="state.message" />
        </div>

        <template v-else>
            <div class="page">
                <VoteInput v-if="state.onNewVote || !state.currentVote" @on-created="createVote"></VoteInput>
                <div v-else>
                    <VoteShow :vote="state.currentVote"></VoteShow>
                    <button @click="onNewVoteClick">Nouveau vote</button>
                </div>
            </div>

            <Footer @terminate="onTerminateClick" />
            <Loading v-if="state.connecting"></Loading>
        </template>
    </Layout>
</template>

<script setup lang="ts">
import { onMounted, reactive, ref, watchEffect } from 'vue';
import { wsConnectTo } from '../shared/ws';
import type { Stats, VoteAdmin, VoteClient } from '../shared/types';
import Loading from '../shared/Loading.vue';
import VoteInput from './VoteInput.vue';
import VoteShow from './VoteShow.vue';
import Header from '../shared/Header.vue';
import InscriptionToggle from './InscriptionToggle.vue';
import { ereaseCookie, getMeInfo } from '../shared/me.ts';
import Footer from './Footer.vue';
import Notice from '../shared/Notice.vue'
import Layout from '../shared/Layout.vue';

interface RoomCreatedState {
    scene: "room-created",
    roomId: number,
    joinable: boolean,
    currentVote: VoteAdmin | null,
    participantsCount: number,
    onNewVote: boolean
    connecting: boolean,
    send: (response: any) => void
}

type State = { scene: "room-creation", message: string } | RoomCreatedState

const createRoom = async () => {
    try {
        // will create a token corresponding to the room
        const ress = await fetch(`/rooms/create`, { method: "POST" })
            .then(a => a.json())

        return ress.roomId as number
    } catch (e) {
        return -1
    }
}

const createRoomCreatedState = (roomId: number): RoomCreatedState => {
    const state: RoomCreatedState = reactive({
        scene: "room-created",
        roomId,
        joinable: true,
        currentVote: null,
        participantsCount: 0,
        onNewVote: true,
        connecting: true,
        send: () => { },
    })

    const { status, send } = wsConnectTo("/admin-ws", (packet: VoteClient | Stats | { open: boolean }) => {
        if ("participantsCount" in packet) {
            state.participantsCount = packet.participantsCount
            if (!state.currentVote) return

            for (let k = 0; k < state.currentVote.answers.length; k++) {
                state.currentVote.answers[k].votes = packet.stats[k] || 0
            }
        } else if ("open" in packet) {
            state.joinable = packet.open
        } else {
            state.currentVote = {
                title: packet.title,
                answers: packet.answers.map(a => ({ message: a, votes: 0 })),
            }
            state.onNewVote = false
        }
    })

    state.send = send
    watchEffect(() => state.connecting = status.value !== "OPEN")

    return state
}

const defaultState: State = { scene: "room-creation", message: "" }
const getInitialState = async (): Promise<State> => {
    const hasCookie = document.cookie.indexOf("auth=") !== -1
    if (!hasCookie) return defaultState

    const meInfo = await getMeInfo()
    if (!meInfo) return defaultState

    if (!meInfo.admin) {
        window.location.assign("/")
        return defaultState
    }

    return createRoomCreatedState(meInfo.roomId)
}

const state = ref<State>(defaultState)
onMounted(async () => state.value = await getInitialState())

const onCreateRoomBtnClick = async () => {
    if (state.value.scene == "room-created") return

    const roomId = await createRoom()
    if (roomId < 0) {
        state.value.message = "Erreur interne"
        return
    }

    state.value = createRoomCreatedState(roomId)
}

const createVote = (vote: Omit<VoteClient, "id">) => {
    if (state.value.scene != "room-created") return
    state.value.send(vote)
    state.value.currentVote = {
        title: vote.title,
        answers: vote.answers.map(msg => ({ message: msg, votes: 0 }))
    }
    state.value.onNewVote = false
}

const onNewVoteClick = () => {
    if (state.value.scene != "room-created") return
    state.value.onNewVote = true
}

const onRoomOpenChange = (open: boolean) => {
    if (state.value.scene != "room-created") return
    state.value.send({ open })
}

const onTerminateClick = () => {
    if (state.value.scene != "room-created") return
    ereaseCookie()
    state.value.send({ terminate: true })
    state.value = { scene: "room-creation", message: "Séance terminée" }
}
</script>

<style>
.room-creation-scene {
    display: flex;
    align-items: center;
    justify-content: center;
    overflow-y: auto;
}

.participant-count {
    text-align: end;
}
</style>
