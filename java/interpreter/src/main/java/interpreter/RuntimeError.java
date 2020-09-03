package interpreter;

public class RuntimeError extends RuntimeException {
  // Some stupid-ass serial number
  private static final long serialVersionUID = -7105974383136136954L;

  final Token token;

  RuntimeError(Token token, String message) {
    super(message);
    this.token = token;
  }

}
