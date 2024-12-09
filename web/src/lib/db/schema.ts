import { relations, SQL, sql } from 'drizzle-orm';
import { sqliteTable, text, int, type AnySQLiteColumn, uniqueIndex } from 'drizzle-orm/sqlite-core';

function lower(email: AnySQLiteColumn): SQL {
	return sql`lower(${email})`;
}

export const projects = sqliteTable(
	'projects',
	{
		id: int().primaryKey({ autoIncrement: true }),
		organization: text().notNull(),
		name: text().notNull(),
		createdAt: text()
			.notNull()
			.default(sql`CURRENT_TIMESTAMP`),
		updatedAt: text()
			.notNull()
			.default(sql`CURRENT_TIMESTAMP`)
	},
	(table) => ({
		// Compound unique constraint
		orgNameIdx: uniqueIndex('projectUniqueIdx').on(lower(table.organization), lower(table.name))
	})
);

export const changelogs = sqliteTable('changelogs', {
	id: int().primaryKey({ autoIncrement: true }),
	projectId: int()
		.notNull()
		.references(() => projects.id, { onDelete: 'cascade' }),
	title: text().notNull(),
	content: text().notNull(),
	isDraft: int({ mode: 'boolean' }).notNull().default(true),
	version: text(),
	createdAt: text()
		.notNull()
		.default(sql`CURRENT_TIMESTAMP`),
	updatedAt: text()
		.notNull()
		.default(sql`CURRENT_TIMESTAMP`)
});

export const changelogProjectRelation = relations(changelogs, ({ one }) => ({
	project: one(projects, {
		fields: [changelogs.projectId],
		references: [projects.id],
		relationName: 'project'
	})
}));
