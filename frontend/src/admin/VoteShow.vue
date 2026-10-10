<template>
    <div class="vote">
        <h3>{{ vote.title }}</h3>

        <ol>
            <AnswerShow 
                v-for="answer in vote.answers" 
                :message="answer.message" 
                :votes="answer.votes" 
                :total-votes="totalVote" />
        </ol>
    </div>
</template>

<script lang="ts" setup>
import { computed } from 'vue';
import type { VoteAdmin } from '../shared/types';
import AnswerShow from './AnswerShow.vue';

const props = defineProps<{ vote: VoteAdmin }>()
const totalVote = computed(() => props.vote.answers.reduce((prev, a) => prev + a.votes, 0))
</script>

<style scoped>
.vote ol {
    padding: 0;
}
</style>
