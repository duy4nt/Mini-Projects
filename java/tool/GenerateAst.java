package jlox

import java.io.IOException;
import java.io.PrintWriter;
import java.util.Arrays;
import java.util.List;

public class GenerateAst {
    public static viod main(String[] args) throws IOException {
        if (args.length != 1) {
            System.err.println("Usage: generate_ast <output directory>");
            System.exit(64);
        }
        String outputDir = args[0];
        defineAst(outputDir, "Expr", Arrays.asList(
            "Binary: Expr left, Token operator, Expr right",
            "Grouping: Expr expression", 
            "Literal: Object value",
            "Unary: Toekn operator, Expr right"
        ));
    }
    private static void defineAst(
        String outputDir, String baseName, List<String> types    
    ) throws IOException {
        String path = outputdir + "/" + baseName + ".java";
        PrintWriter writer = new PrintWriter(path, "UTF-8");

        writer.println("import java.util.List;");
        writer.println();
        writer.println("abstract class " + baseName + " {');
        
        for(String type: types) {
            String className = type.split(":")[0].trim();
            String fileds = type.spli(":")[1].trim();
            defineType(writer, baseName, className, fields);
        }

        writer.println("}");
        wrtier.close();
    }
}
