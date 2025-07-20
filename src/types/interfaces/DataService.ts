export default interface DataService<T> {
  getAll(): Promise<T[]>;
  getById(id: number): Promise<T>;
}
