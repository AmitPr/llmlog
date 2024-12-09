import type { PageServerLoad } from './$types';
import { db } from '$lib/db';
import { changelogs } from '$lib/db/schema';
import { z } from 'zod';
import { and, eq } from 'drizzle-orm';
import { error, redirect } from '@sveltejs/kit';

const loadChangelogSchema = z.object({
	id: z.coerce.number().int().positive()
});

export const load: PageServerLoad = async ({ params }) => {
	let id: number;
	try {
		({ id } = loadChangelogSchema.parse(params));
	} catch (e) {
		if (e instanceof z.ZodError) {
			error(400, 'Malformed request parameters');
		}
		error(500, 'An unknown error occurred');
	}

	const changelog = await db.query.changelogs.findFirst({
		where: and(eq(changelogs.id, id)),
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
		changelog.project.organization !== params.org ||
		changelog.project.name !== params.repo
	) {
		error(404, 'Changelog not found');
	}

	if (!changelog.isDraft) {
		const { org, repo } = params;
		throw redirect(301, `/${org}/${repo}/${id}`);
	}

	return {
		changelog
	};
};
