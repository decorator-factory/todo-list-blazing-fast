import * as z from "zod"

export const apiErrorSchema = z.object({
    status: z.literal("error"),
    error: z.string(),
    detail: z.any(),
})

export type ApiError = z.infer<typeof apiErrorSchema>

export type ApiAnswer<T> = { status: "ok"; payload: T } | ApiError

export function apiAnswer<T>(schema: z.Schema<T>) {
    return z.discriminatedUnion("status", [
        z.object({ status: z.literal("ok"), payload: schema }),
        apiErrorSchema,
    ])
}

export const todoItemOutSchema = z.object({
    id: z.number(),
    title: z.string(),
    done: z.boolean(),
})

export type TodoItemOut = z.infer<typeof todoItemOutSchema>

export async function fetchTodos(
    baseUrl: string,
    signal: AbortSignal | null = null,
): Promise<ApiAnswer<TodoItemOut[]>> {
    const json = await fetch(`${baseUrl}/todos`, { signal }).then((r) =>
        r.json(),
    )
    return apiAnswer(z.array(todoItemOutSchema)).parse(json)
}

export async function markTodo(
    baseUrl: string,
    id: number,
    signal: AbortSignal | null = null,
): Promise<ApiAnswer<null>> {
    const json = await fetch(`${baseUrl}/todos/${id}/mark`, {
        method: "POST",
        signal,
    }).then((r) => r.json())
    return apiAnswer(z.null()).parse(json)
}

export async function unmarkTodo(
    baseUrl: string,
    id: number,
    signal: AbortSignal | null = null,
): Promise<ApiAnswer<null>> {
    const json = await fetch(`${baseUrl}/todos/${id}/unmark`, {
        method: "POST",
        signal,
    }).then((r) => r.json())
    return apiAnswer(z.null()).parse(json)
}

export async function deleteTodo(
    baseUrl: string,
    id: number,
    signal: AbortSignal | null = null,
): Promise<ApiAnswer<null>> {
    const json = await fetch(`${baseUrl}/todos/${id}`, {
        method: "DELETE",
        signal,
    }).then((r) => r.json())
    return apiAnswer(z.null()).parse(json)
}

export async function createTodo(
    baseUrl: string,
    title: string,
    signal: AbortSignal | null = null,
): Promise<ApiAnswer<number>> {
    const json = await fetch(`${baseUrl}/todos`, {
        method: "POST",
        signal,
        body: JSON.stringify({ title }),
        headers: { "Content-Type": "application/json" },
    }).then((r) => r.json())
    return apiAnswer(z.number()).parse(json)
}
