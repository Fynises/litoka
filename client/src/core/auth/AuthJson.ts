export interface AuthSuccessJson {
  code: string,
  scope: string,
  state: string,
}

export interface AuthErrorJson {
  error: string,
  error_description: string,
  state: string,
}

export default class AuthJson {

  searchParams: URLSearchParams;

  constructor(searchParams: URLSearchParams) {
    this.searchParams = searchParams;
  }

  private tryGet(query: string): string {
    const param = this.searchParams.get(query);
    if (param !== null) {
      return param as string;
    } else {
      return '';
    }
  }

  getSuccessJson(): AuthSuccessJson {
    const authSuccessJson: AuthSuccessJson = {
      code: this.tryGet('code'),
      scope: this.tryGet('scope'),
      state: this.tryGet('state')
    };
    return authSuccessJson;
  }

  getErrorJson(): AuthErrorJson {
    const authErrorJson: AuthErrorJson = {
      error: this.tryGet('error'),
      error_description: this.tryGet('error_description'),
      state: this.tryGet('state')
    };
    return authErrorJson;
  }

}