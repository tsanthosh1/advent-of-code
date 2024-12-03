
export async function readInput(day: string): Promise<string[]> {
    const content = await Deno.readTextFile(`../${day}/input.txt`);
    return content.trim().split("\n");
}
