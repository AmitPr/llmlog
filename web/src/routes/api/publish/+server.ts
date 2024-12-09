// src/routes/api/create/+page.server.ts
import { json, type RequestHandler } from '@sveltejs/kit';
import { db } from '$lib/db';
import { z } from 'zod';
import { changelogs } from '$lib/db/schema';
import { eq } from 'drizzle-orm';

const publishChangelogSchema = z.object({
	id: z.coerce.number().int().positive()
});

export const POST: RequestHandler = async ({ request }) => {
	try {
		const rawData = await request.json();
		const data = publishChangelogSchema.parse(rawData);

		let changelog = await db.query.changelogs.findFirst({
			where: eq(changelogs.id, data.id)
		});
		if (!changelog) {
			return json({ error: 'Changelog not found' }, { status: 404 });
		}
		if (!changelog.isDraft) {
			return json({ error: 'Changelog is already published' }, { status: 400 });
		}

		const updated = await db
			.update(changelogs)
			.set({
				isDraft: false
			})
			.where(eq(changelogs.id, data.id))
			.returning();
		changelog = updated[0];

		return json({ changelog });
	} catch (error) {
		if (error instanceof z.ZodError) {
			return json({ error: error.issues }, { status: 400 });
		}
		return json({ error: 'Failed to publish changelog' }, { status: 500 });
	}
};
