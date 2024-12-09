<script lang="ts">
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
</script>

<div class="flex justify-between mb-6">
	<div class="flex gap-2">
		<p>
			<span class="subdued">{data.project?.organization} /</span>
			<span class="font-semibold">{data.project?.name}</span>
		</p>
	</div>
</div>

<main class="flex flex-col gap-6">
	{#each data.changelogs as changelog}
		<a
			href="/{data.project?.organization}/{data.project?.name}/{changelog.id}"
			class="group flex flex-col py-2 rounded-lg hover:bg-gray-100 transition-colors"
		>
			<h2 class="text-2xl font-mono font-bold group-hover:text-blue-600 transition-colors">
				{changelog.title}
			</h2>
			<span class="text-xs subdued font-semibold">
				{new Date(changelog.createdAt).toLocaleDateString()}
			</span>
			<p class="text-sm subdued line-clamp-2">
				<!-- Sanitized in SSR -->
				<!-- eslint-disable-next-line svelte/no-at-html-tags -->
				{@html changelog.content}
			</p>
		</a>
	{/each}
</main>
