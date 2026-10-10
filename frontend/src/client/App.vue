<template>
    <Layout>
        <Header :code="state.scene == 'room' ? state.roomId : null"></Header>
    
        <div v-if="state.scene == 'code'" class="code-scene">
            <div>
                <p>Rentrez le code de la séance : </p>
                <RoomIdInput @complete="onRoomIdInputComplete"></RoomIdInput>
            </div>
            <Notice :message="state.notice" />
        </div>

        <template v-else>
            <div  class="page">
                <p v-if="!state.currentVote">En attente de vote ...</p>
                <template v-else>
                    <h3>{{ state.currentVote.title }}</h3>
                    <Vote :answers="state.currentVote.answers" @select="onAnswerSelect" :key="state.currentVote.id"/>
                </template>
            </div>

            <Footer @quit="quit"></Footer>
                <Loading v-if="state.connecting"></Loading>
        </template>
    </Layout>
</template>

<script setup lang="ts">
import { onMounted, reactive, ref, watchEffect } from 'vue'
import { wsConnectTo } from '../shared/ws'
import RoomIdInput from './RoomIdInput.vue'
import Vote from './Vote.vue'
import Loading from '../shared/Loading.vue'
import type { VoteClient } from '../shared/types.ts'
import Header from '../shared/Header.vue'
import { ereaseCookie, getMeInfo } from '../shared/me.ts'
import Notice from '../shared/Notice.vue'
import Layout from '../shared/Layout.vue'
import Footer from './Footer.vue'

interface CodeState {
    scene: "code",
    notice: string,
}

interface RoomState {
    scene: "room",
    roomId: number,
    connecting: boolean,
    currentVote?: VoteClient,
    send: (response: any) => void
}

type State = CodeState | RoomState

const joinRoom = async (roomId: number) => {
    try {
        // will create a token corresponding to the room
        const data = await fetch(`/rooms/${roomId}/join`, {
            method: "POST",
        })

        return data.status == 200
    } catch (e) {
        return false
    }
}

const onRoomIdInputComplete = async (id: number) => {
    if (state.value.scene != "code") return
    state.value.notice = ""

    if (await joinRoom(id)) {
        state.value = createRoomCtx(id)
        return
    }

    state.value.notice = "Code invalide"
}

const createRoomCtx = (roomId: number) => {
    const roomState: RoomState = reactive({
        scene: "room",
        roomId,
        connecting: true,
        send: () => { },
    })

    const { status, send } = wsConnectTo("/ws", (packet: VoteClient | { terminate: true }) => {
        if ("terminate" in packet) {
            ereaseCookie()
            state.value = { scene: 'code', notice: "Séance terminée" }
            return
        }

        roomState.currentVote = packet
    })

    roomState.send = send
    watchEffect(() => roomState.connecting = status.value !== "OPEN")

    return roomState
}

const defaultState: State = { scene: "code", notice: "" } 
const getInitialState = async (): Promise<State> => {
    const hasCookie = document.cookie.indexOf("auth=") !== -1
    if (!hasCookie) return defaultState

    const meInfo = await getMeInfo()
    if (!meInfo) return defaultState

    if (meInfo.admin) {
        window.location.assign("/admin")
        return defaultState
    }

    return createRoomCtx(meInfo.roomId)
}

const state = ref<State>(defaultState)
onMounted(async () => state.value = await getInitialState())

const onAnswerSelect = (answerId: number) => {
    if (state.value.scene != "room" || !state.value.currentVote) return

    state.value.send({
        voteId: state.value.currentVote.id,
        answerId,
    })
}

const quit = () => {
    ereaseCookie()
    state.value = { scene: 'code', notice: "Séance quitté" }
}
</script>

<style>
.code-scene {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100vh;
}
</style>
