static void TpInfoPlus()
{
    Console.Write("Comment vous appelez vous (nom prenom)?\n> ");
    string nom_prenom = Console.ReadLine();

    Console.Write("Saisissez votre année de naissance\n> ");
    int annee = Convert.ToInt32(Console.ReadLine());

    Console.Write("Saisissez votre taille\n> ");
    double taille = Convert.ToDouble(Console.ReadLine());
    Console.Write("Saisissez votre poids\n> ");
    double poids = Convert.ToDouble(Console.ReadLine());


    string[] nom_prenom_arr = nom_prenom.Split(' ');
    string prenom = nom_prenom_arr[1].ToUpper()[0] + nom_prenom_arr[1].Substring(1).ToLower();
    string nom = nom_prenom_arr[0].ToUpper();
    int age = (2024 - annee);

    Console.WriteLine($"NOM: {nom}, PRENOM: {prenom}");
    Console.WriteLine($"{age} ans");
    Console.WriteLine($"TAILLE: {taille} metre, POIDS: {poids}kg");
}

// VERSION PLUS EVOLUE

static T ReadValue<T>(string txt)
{
    do
    {
        try
        {
            Console.Write($"{txt}\n> ");
            return (T)Convert.ChangeType(Console.ReadLine(), typeof(T));
        }
        catch (FormatException e)
        {
            Console.WriteLine($"Mauvaise valeur entrée: {e}");
        }
    } while (true);
}
static void TpInfoPlus1()
{
    string nom_prenom = ReadValue<string>("Comment vous appelez vous (nom prenom)?");
    int annee = ReadValue<int>("Saisissez votre année de naissance");
    double taille = ReadValue<double>("Saisissez votre taille");
    double poids = ReadValue<double>("Saisissez votre poids");

    string[] nom_prenom_arr = nom_prenom.Split(' ');
    string prenom = nom_prenom_arr[1].ToUpper()[0] + nom_prenom_arr[1].Substring(1).ToLower();
    string nom = nom_prenom_arr[0].ToUpper();
    int age = (2024 - annee);

    Console.WriteLine($"NOM: {nom}, PRENOM: {prenom}");
    Console.WriteLine($"{age} ans");
    Console.WriteLine($"TAILLE: {taille} metre, POIDS: {poids}kg");
}