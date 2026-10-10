<template>
    <div class="vote-input">
        <div class="title-wrapper">
            <label for="title">Titre du sondage :</label>
            <input type="text" id="title" v-model="title">
        </div>

        <p>Réponses possibles :</p>
        <ol>
            <li class="answer-input" v-for="(_, index) in answers" :key="index"
                :style="{ opacity: index == answers.length - 1 ? '0.3' : '1' }">
                <input type="text" v-model="answers[index]" />
            </li>
        </ol>

        <div class="btns">
            <button @click="validate">Valider</button>
        </div>
    </div>
</template>

<script lang="ts" setup>
import type { VoteClient } from '../shared/types'
import { ref, watch, nextTick } from 'vue'

const emits = defineEmits<{
    (event: "onCreated", vote: Omit<VoteClient, "id">): void
}>()

const title = ref("")
const answers = ref<string[]>([''])

let updating = false

watch(
    answers,
    async (currentAnswers) => {
        if (updating) return

        updating = true

        // Supprime les lignes vides en trop à la fin.
        while (
            currentAnswers.length > 1 &&
            currentAnswers[currentAnswers.length - 1] === "" &&
            currentAnswers[currentAnswers.length - 2] === ""
        ) {
            currentAnswers.pop()
        }

        // Ajoute une ligne vide si la dernière est remplie.
        if (currentAnswers[currentAnswers.length - 1] !== "") {
            currentAnswers.push("")
        }

        await nextTick()
        updating = false
    },
    { deep: true, flush: 'sync' }
)

const validate = () => {
    const validAnswers = answers.value.filter(
        answer => answer.trim().length > 0
    )

    emits("onCreated", {
        title: title.value,
        answers: validAnswers
    })
}
</script>

<style scoped>
.answer-input:focus-within {
    opacity: 1 !important;
}

.title-wrapper {
    margin-bottom: 16px;
}
</style>
