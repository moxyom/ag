<template>
    <main>
        <slot></slot>
    </main>
</template>

<style>
main {
    height: 100vh;
    display: flex;
    flex-direction: column;
}

main>div {
    flex: 1;
    overflow-y: auto;
}
</style>
