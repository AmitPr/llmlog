import type { PageServerLoad } from './$types';
import { db } from '$lib/db';
import { changelogs, projects } from '$lib/db/schema';
import { z } from 'zod';
import { and, eq, desc } from 'drizzle-orm';
import { marked } from 'marked';
import { JSDOM } from 'jsdom';
import DOMPurify from 'dompurify';
import { error } from '@sveltejs/kit';

const { window } = new JSDOM();
const purify = DOMPurify(window);

const ITEMS_PER_PAGE = 10;

const projectParamsSchema = z.object({
	org: z.string(),
	repo: z.string()
});

export const load: PageServerLoad = async ({ params }) => {
	let org, repo;
	try {
		({ org, repo } = projectParamsSchema.parse(params));
	} catch (e) {
		if (e instanceof z.ZodError) {
			error(400, 'Malformed request parameters');
		}
		error(500, 'An unknown error occurred');
	}

	const project = await db.query.projects.findFirst({
		where: and(eq(projects.organization, org), eq(projects.name, repo))
	});

	if (!project) {
		error(404, 'Project not found');
	}

	const page = await db.query.changelogs.findMany({
		where: and(eq(changelogs.projectId, project.id), eq(changelogs.isDraft, false)),
		orderBy: desc(changelogs.createdAt),
		limit: ITEMS_PER_PAGE
	});

	const rendered = await Promise.all(
		page.map(async (changelog) => {
			changelog.content = purify.sanitize(await marked.parse(changelog.content));
			return changelog;
		})
	);

	//TODO: Pagination

	return {
		project,
		changelogs: rendered,
		itemsPerPage: ITEMS_PER_PAGE
	};
};
