<template>
    <Section is="header">
        <CodeShow v-if="code" :code="code"></CodeShow>
        <p v-else>Pas de séance en cours</p>

        <div>
            <slot></slot>
        </div>
    </Section>
</template>

<script lang="ts" setup>
import CodeShow from './CodeShow.vue';
import Section from './Section.vue';
defineProps<{ code: number | null }>()
</script>
