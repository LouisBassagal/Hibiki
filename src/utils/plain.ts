export const plain = <T>(value: T): T => JSON.parse(JSON.stringify(value))
