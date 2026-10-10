<template>
    <Section is="footer">
        <div></div>

        <button @click="emits('terminate')">Terminer la séance</button>
    </Section>
</template>

<script lang="ts" setup>
import Section from '../shared/Section.vue';

const emits = defineEmits<{
    (event: "terminate"): void
}>()
</script>
