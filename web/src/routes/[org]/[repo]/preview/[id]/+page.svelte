<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { Editor } from '@tiptap/core';
	import { Markdown } from 'tiptap-markdown';
	import StarterKit from '@tiptap/starter-kit';
	import type { PageData } from './$types';
	import { goto } from '$app/navigation';

	let { data }: { data: PageData } = $props();

	let element: HTMLDivElement;
	let editor: Editor;
	let content = $state(data.changelog?.content);

	const debounce = (fn: () => void | Promise<void>, ms: number) => {
		let timeout: number;
		return function () {
			clearTimeout(timeout);
			timeout = setTimeout(fn, ms);
		};
	};

	const save = debounce(async () => {
		const updateRequest = {
			id: data.changelog.id,
			title: title,
			content: editor.storage.markdown.getMarkdown()
		};

		if (
			updateRequest.title === data.changelog.title &&
			updateRequest.content === data.changelog.content
		) {
			return;
		}

		const response = await fetch(`/api/update`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(updateRequest)
		});

		console.log(response);
	}, 500);

	onMount(() => {
		editor = new Editor({
			element: element,
			extensions: [StarterKit, Markdown],
			content: content,
			onTransaction: () => {
				editor = editor;
				save();
			}
		});
	});

	onDestroy(() => {
		if (editor) {
			editor.destroy();
		}
	});

	let title = $state(data.changelog?.title || '');
	$inspect(title);

	async function publish(id: number) {
		const response = await fetch(`/api/publish`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				id
			})
		});

		if (response.ok) {
			const json = await response.json();
			console.log(json);
			goto(`../${id}`);
		}
	}
</script>

<div class="flex justify-between">
	<div class="flex gap-2">
		<p><span class="subdued">{data.org} /</span> <span class="font-semibold">{data.repo}</span></p>
		{#if data.changelog}
			<pre class="dark font-bold px-0.5 rounded-md">{data.changelog.id}</pre>
		{/if}
	</div>
	{#if data.changelog}
		<div>
			<button class="btn btn-primary" onclick={() => publish(data.changelog.id)}>Publish</button>
		</div>
	{/if}
</div>
<main class="flex flex-col gap-2">
	<div class="flex flex-col">
		<label for="title" class="uppercase text-xs subdued">Release Title</label>
		<div
			class="w-full text-5xl font-mono font-bold bg-transparent outline-none break-words"
			id="title"
			contenteditable
			bind:textContent={title}
			onkeypress={(e) => {
				if (e.key === 'Enter') {
					e.preventDefault();
				}
			}}
			role="textbox"
			aria-multiline="false"
			tabindex="0"
		>
			{title}
		</div>
	</div>
	<div class="flex flex-col">
		<p class="subdued uppercase text-xs">Release Notes</p>
		<div bind:this={element} id="editor"></div>
	</div>
</main>
