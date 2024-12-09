<script lang="ts">
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
</script>

<div class="flex justify-between">
	<div class="flex gap-2">
		<a href="/{data.org}/{data.repo}"
			><span class="subdued">{data.org} /</span> <span class="font-semibold">{data.repo}</span></a
		>
	</div>
</div>
<main class="flex flex-col gap-2">
	<div class="flex flex-col">
		<h1 class="w-full text-5xl font-mono font-bold bg-transparent break-words">
			{data.changelog?.title}
		</h1>
	</div>
	<div class="flex flex-col">
		<div id="editor">
			<!-- Sanitized in SSR -->
			<!-- eslint-disable-next-line svelte/no-at-html-tags -->
			{@html data.changelog?.content}
		</div>
	</div>
</main>
