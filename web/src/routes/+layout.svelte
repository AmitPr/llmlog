<script lang="ts">
	import '../app.css';

	let { children } = $props();
</script>

<div class="mx-auto max-w-prose">
	<div class="mx-4 mt-4">
		{@render children()}
	</div>
</div>

<style lang="postcss">
	:global(html) {
		@apply subpixel-antialiased bg-slate-100;
	}
</style>
