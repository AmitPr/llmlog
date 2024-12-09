import type { PageServerLoad } from './$types';
import { db } from '$lib/db';
import { changelogs } from '$lib/db/schema';
import { z } from 'zod';
import { eq } from 'drizzle-orm';
import { marked } from 'marked';
import { JSDOM } from 'jsdom';
import DOMPurify from 'dompurify';
import { error } from '@sveltejs/kit';

const { window } = new JSDOM();
const purify = DOMPurify(window);

const changelogIdSchema = z.object({
	id: z.coerce.number().int().positive()
});

export const load: PageServerLoad = async ({ params }) => {
	let id: number;
	try {
		({ id } = changelogIdSchema.parse(params));
	} catch (e) {
		if (e instanceof z.ZodError) {
			error(400, 'Malformed request parameters');
		}
		error(500, 'An unknown error occurred');
	}

	const changelog = await db.query.changelogs.findFirst({
		where: eq(changelogs.id, id),
		with: {
			project: {
				columns: {
					organization: true,
					name: true
				}
			}
		}
	});

	if (
		!changelog ||
		changelog.isDraft ||
		changelog.project.organization !== params.org ||
		changelog.project.name !== params.repo
	) {
		error(404, 'Changelog not found');
	}

	const content = purify.sanitize(await marked.parse(changelog.content));
	changelog.content = content;

	return {
		changelog
	};
};
