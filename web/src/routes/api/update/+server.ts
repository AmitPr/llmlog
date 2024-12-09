// src/routes/api/create/+page.server.ts
import { json, type RequestHandler } from '@sveltejs/kit';
import { db } from '$lib/db';
import { z } from 'zod';
import { changelogs } from '$lib/db/schema';
import { eq } from 'drizzle-orm';

const updateChangelogSchema = z.object({
	id: z.coerce.number().int().positive(),
	title: z.string().min(1).optional(),
	content: z.string().min(1).optional()
});

export const POST: RequestHandler = async ({ request }) => {
	try {
		const rawData = await request.json();
		const data = updateChangelogSchema.parse(rawData);

		const changelog = await db
			.update(changelogs)
			.set({
				title: data.title,
				content: data.content
			})
			.where(eq(changelogs.id, data.id))
			.returning();

		return json({ changelog: changelog[0] });
	} catch (error) {
		if (error instanceof z.ZodError) {
			return json({ error: error.issues }, { status: 400 });
		}
		return json({ error: 'Failed to update changelog' }, { status: 500 });
	}
};
