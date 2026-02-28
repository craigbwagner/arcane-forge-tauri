export default interface DataService<T> {
	getAll(): Promise<T[]>;
	getById(id: number): Promise<T>;
	create(): Promise<T>;
	update(data: T): Promise<T>;
	delete(id: number): Promise<boolean>;
}
