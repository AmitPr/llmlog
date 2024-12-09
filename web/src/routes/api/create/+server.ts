// src/routes/api/create/+page.server.ts
import { json, type RequestHandler } from '@sveltejs/kit';
import { db } from '$lib/db';
import { z } from 'zod';
import { changelogs, projects } from '$lib/db/schema';
import { and, eq } from 'drizzle-orm';

const createChangelogSchema = z.object({
	organization: z.string().min(1),
	name: z.string().min(1),
	title: z.string().min(1),
	content: z.string().min(1)
});

export const POST: RequestHandler = async ({ request }) => {
	try {
		const rawData = await request.json();
		const data = createChangelogSchema.parse(rawData);

		let project = await db.query.projects.findFirst({
			where: and(eq(projects.organization, data.organization), eq(projects.name, data.name))
		});

		if (!project) {
			const inserted = await db
				.insert(projects)
				.values({
					organization: data.organization,
					name: data.name
				})
				.returning();
			project = inserted[0];
		}

		const changelog = await db
			.insert(changelogs)
			.values({
				projectId: project.id,
				title: data.title,
				content: data.content
			})
			.returning();

		return json({ project, changelog: changelog[0] });
	} catch (error) {
		if (error instanceof z.ZodError) {
			return json({ error: error.issues }, { status: 400 });
		}
		return json({ error: 'Failed to create project' }, { status: 500 });
	}
};
