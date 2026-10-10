<template>
<p class="code">
    <span>{{ strParts[0] }}</span>
    -
    <span>{{ strParts[1].toString().padStart(3, "0") }}</span>
</p>
</template>

<script lang="ts" setup>
import { computed } from "vue"
 
const props = defineProps<{ code: number }>()
const strParts = computed(() => [Math.floor(props.code / 1000), props.code % 1000])
</script>

<style scoped>
.code {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 2px;
    font-size: larger;
    color: black;
}

.code span {
    padding: 4px;
    background: var(--color-primary-transparent);
    border-radius: 6px;
    line-height: 1em;
    letter-spacing: 0.1em;
    font-style: italic;
}
</style>
