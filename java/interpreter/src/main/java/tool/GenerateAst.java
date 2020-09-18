// run using
// javac .\GenerateAst.java ; java GenerateAst ..\interpreter ;
package tool;

import java.io.IOException;
import java.io.PrintWriter;
import java.util.Arrays;
import java.util.List;

public class GenerateAst {
  public static void main(final String[] args) throws IOException {
    if (args.length != 1) {
      System.err.println("Usage: generate_ast <output_directory>");
      System.exit(64);
    }

    final String outputDir = args[0];

    defineAst(outputDir, "Expr",
        Arrays.asList("Assign : Token name, Expr value", "Binary : Expr left, Token operator, Expr right",
            "Grouping : Expr expressions", "Literal : Object value", "Unary: Token operator, Expr right",
            "Variable: Token name"));
    defineAst(outputDir, "Stmt", Arrays.asList("Block : List<Stmt> statements", "Expression : Expr expression",
        "Print : Expr expression", "Var : Token name, Expr initializer"));

  }

  private static void defineAst(final String outputDir, final String baseName, final List<String> types)
      throws IOException {
    final String path = outputDir + "/" + baseName + ".java";
    final PrintWriter writer = new PrintWriter(path, "UTF-8");

    writer.println("package interpreter;");
    writer.println();
    writer.println("import java.util.List;");
    writer.println();
    writer.println("public abstract class " + baseName + " {");

    defineVisitor(writer, baseName, types);

    // The AST classes
    for (final String type : types) {
      final String[] typesAsArr = type.split(":");
      final String className = typesAsArr[0].trim();
      final String fields = typesAsArr[1].trim();
      defineType(writer, baseName, className, fields);
    }

    // the base accept() method
    writer.println();
    writer.println("  abstract <R> R accept(Visitor<R> visitor);");

    writer.println("}");
    writer.close();

  }

  private static void defineVisitor(final PrintWriter writer, final String baseName, final List<String> types) {
    writer.println("  interface Visitor<R> {");

    for (final String type : types) {
      final String typeName = type.split(":")[0].trim();
      writer.println("  R visit" + typeName + baseName + "(" + typeName + " " + baseName.toLowerCase() + ");");
    }

    writer.println("  }");
  }

  private static void defineType(final PrintWriter writer, final String baseName, final String className,
      final String fieldList) {
    writer.println(" static class " + className + " extends " + baseName + " {");

    // Constructor
    writer.println("  " + className + "(" + fieldList + ") {");

    // Store parameters in fields
    final String[] fields = fieldList.split(", ");
    for (final String field : fields) {
      final String name = field.split(" ")[1];
      writer.println("    this." + name + " = " + name + ";");
    }

    writer.println("  }");

    // Visitor Pattern
    writer.println();
    writer.println(" @Override");
    writer.println("  <R> R accept(Visitor<R> visitor) {");
    writer.println("    return visitor.visit" + className + baseName + "(this);");
    writer.println("  }");

    // Fields.
    writer.println();
    for (final String field : fields) {
      writer.println("  final " + field + ";");
    }

    writer.println("  }");
  }
}
