<script lang="ts">
    let x = 0;
    let y = 0;
    let scale = 1;

    function handlePointerMove(event: PointerEvent) {
        x = event.clientX / window.innerWidth - 0.5;
        y = event.clientY / window.innerHeight - 0.5;
        const distance = Math.sqrt(x * x + y * y);
        scale = 1 + (1 - distance) / 5;
    }
</script>

<div on:pointermove={handlePointerMove} class="bg-surface-900 w-full h-full rounded-lg bg-opacity-50 flex flex-col items-center justify-center">
    <h1 class="h1" style="transform: translate3d({x * 20}px, {y * 20}px, 0) scale({scale});">
        <span class="gradient-heading text-9xl cursor-default select-none">Hubio.</span>
    </h1>
</div>