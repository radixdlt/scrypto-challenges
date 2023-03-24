export class HttpError extends Error {
    constructor(public status: number, message: string) {
      super(message)
    }
  }
  
  export class BadRequestError extends HttpError {
    constructor(message: string) {
      super(400, message)
    }
  }
  
  export class NotFoundError extends HttpError {
    constructor(message: string) {
      super(404, message)
    }
  }
  
  export class InternalServerError extends HttpError {
    constructor(message: string) {
      super(500, message)
    }
  }
