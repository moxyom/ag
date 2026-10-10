<template>
    <li class="answer" :style="{ '--pr': pr, '--pr-full': prFull }">
        <p ref="message">{{ message }}</p>
        <p class="number">{{ votes }}</p>
    </li>
</template>

<script lang="ts" setup>
import { useResizeObserver } from '@vueuse/core';
import { computed, ref, useTemplateRef } from 'vue'

const props = defineProps<{
    totalVotes: number,
    votes: number,
    message: string
}>()

const textRef = useTemplateRef("message")
const textWidth = ref(32)
useResizeObserver(textRef, (entries) => {
    const entry = entries[0]
    const { width } = entry.contentRect
    textWidth.value = width
})

const pr = computed(() => {
    const non0TotalVotes = props.totalVotes != 0 ? props.totalVotes : 1
    return `${(props.votes / non0TotalVotes) * 100}%`
})
const prFull = computed(() => `max(${pr.value}, ${textWidth.value + 32 + 8}px)`)
</script>

<style scoped>
.answer p {
    width: fit-content;
}
.answer {
    list-style-type: none;
    margin: 0;
    position: relative;
    margin: 8px 0;
    padding: 0 12px;
    color: black;
}

.answer::before {
    content: "";
    width: max(var(--pr), 8px);
    height: 100%;
    top: 0;
    left: 0;
    position: absolute;
    z-index: -1;

    background: var(--color-primary);
    opacity: 0.2;
    border-radius: 2px;
    transition: width 0.4s var(--cubic-bounce);
}

.number {
    color: var(--color-primary);
    font-size: large;
    position: absolute;
    top: 50%;
    right: calc(100% - var(--pr-full) + 8px);
    transform: translateY(-50%);
    transition: right 0.4s var(--cubic-bounce);
}
</style>
