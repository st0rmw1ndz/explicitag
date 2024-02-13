using System.Reflection;
using System.Text.RegularExpressions;
using CommandLine;

namespace ExpliciTag
{
    public class Options
    {
        [Value(
            0,
            MetaName = "paths",
            Required = true,
            HelpText = "List of file and/or directory paths."
        )]
        public required IEnumerable<string> Paths { get; set; }
    }

    class Program
    {
        private static string[] GetExplicitWords()
        {
            string[] explicitWords;
            var assembly = Assembly.GetExecutingAssembly();
            using (
                Stream? stream = assembly.GetManifestResourceStream("ExpliciTag.ExplicitWords.txt")
            )
            {
                if (stream == null)
                    return [];

                using var reader = new StreamReader(stream);
                string content = reader.ReadToEnd();
                explicitWords = content.Split(
                    new[] { Environment.NewLine },
                    StringSplitOptions.None
                );
            }

            return explicitWords;
        }

        private static void RunWithOptions(Options opts)
        {
            string[] explicitWords = GetExplicitWords();
            if (explicitWords.Length == 0)
                return;

            string pattern = @"\b(" + string.Join("|", explicitWords) + @")\b";
            Regex explicitPattern = new(pattern, RegexOptions.IgnoreCase);

            foreach (var path in opts.Paths)
            {
                if (File.Exists(path))
                    ProcessFile(path, explicitPattern);
                else if (Directory.Exists(path))
                    ProcessDirectory(path, explicitPattern);
                else
                    Console.WriteLine($"Invalid path: {path}");
            }
        }

        private static void HandleParseError(IEnumerable<Error> errs) { }

        private static void ProcessFile(string filePath, Regex explicitPattern)
        {
            try
            {
                using var file = TagLib.File.Create(filePath);
                if (file.MimeType != "taglib/m4a")
                    return;

                string? lyrics = file.Tag.Lyrics?.ToLower();
                bool isExplicit = explicitPattern.IsMatch(lyrics ?? string.Empty);
                string rating = isExplicit ? "Explicit" : "Clean";

                Console.WriteLine($"File: {filePath}, Rating: {rating}");

                var tags = (TagLib.Mpeg4.AppleTag)file.GetTag(TagLib.TagTypes.Apple);

                TagLib.ByteVector customTagName = "rtng";
                TagLib.ByteVector customTagData = isExplicit ? [1] : [0];
                uint customTagFlag = (uint)TagLib.Mpeg4.AppleDataBox.FlagType.ContainsData;

                tags.SetData(customTagName, customTagData, customTagFlag);

                file.Save();
            }
            catch (Exception e)
            {
                Console.WriteLine(e.Message);
            }
        }

        private static void ProcessDirectory(string directoryPath, Regex explicitPattern)
        {
            var files = Directory.EnumerateFiles(
                directoryPath,
                "*.m4a",
                SearchOption.AllDirectories
            );
            foreach (var file in files)
            {
                ProcessFile(file, explicitPattern);
            }
        }

        static void Main(string[] args)
        {
            Parser
                .Default.ParseArguments<Options>(args)
                .WithParsed(RunWithOptions)
                .WithNotParsed(HandleParseError);
        }
    }
}
