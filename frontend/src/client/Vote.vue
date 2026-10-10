<template>
    <div :class="['vote', choiceDone ? 'choice-done' : '']">
        <div v-for="(answer, index) in answers" :class="['answer', index == selected ? 'selected' : '']"
            @click="() => select(index)">
            {{ answer }}
        </div>
        <div class="btns">
            <button @click="validate" :class="choiceDone || selected == -1 ? 'disable' : ''">valider</button>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'

const props = defineProps<{
    answers: string[]
}>()

const emits = defineEmits<{
    (event: "select", answerId: number): void
}>()

const selected = ref(-1)
const choiceDone = ref(false)

const select = (index: number) => {
    if (choiceDone.value) return
    selected.value = index
}

const validate = () => {
    if (choiceDone.value) return
    emits("select", selected.value)
    choiceDone.value = true
}
</script>

<style scoped>
.answer {
    border: 3px solid var(--color-primary);
    margin: 24px 0;
    padding: 8px 16px;
    border-radius: 2px;
    cursor: pointer;
}

.choice-done .answer:not(.selected) {
    opacity: 0.5;
}

.selected {
    color: white;
    background: var(--color-primary);
}

.btns {
    display: flex;
    justify-content: flex-end;
}
</style>
