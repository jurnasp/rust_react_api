export class AppService {

    public async getUser(): Promise<any> {
        const response = await fetch('/api/user');
        return await response.json();
    }
}
