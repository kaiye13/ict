# LINQ
LINQ staat voor Language-Integrated Query, het is de technologie die zorgt voor query mogelijkheden in C#. Dit betekent dat het mogelijk gemaakt wordt om door middel van een SQL-achtige (aka query-syntax) van een datasource zij het XML, SQL,... informatie te ontrekken rechtstreeks in C#.
## Selectie van element van type T voor collectie van elementen van type T.
Volgende voorbeelden zijn gelijkaardig aan elkander. Allen hebben ze de bedoeling om van een collection van artiesten **1** artiest te extracten en deze terug te geven **aan de hand van de artistId**.

### Met foreach
```C#
foreach (Artist artist in artists)
{
	if (artist.Id == id)
	{
		return artist;
	}
}
```
### Met LINQ 2 (zonder lambda).
```C#
return from artist in artists 
    where artist.Id == id
```
### Met lambda expressie
*Even terzijde* dit gaat nog 'makkelijker' door een lambda expressie.
```C#
return artists.FirstOrDefault(artist => artist.Id == id);
```

## LINQ: de 3 delen
Over het algemeen bestaat een LINQ operatie uit 3 delen,
1) Het ter beschikking stellen van een (ruwe) datasource.
2) De query ontwerpen
3) De query uitvoeren

Dit in een voorbeeld:
```C#
class LINQExample
{        
    static void Main()
    {
        // 1. Data source
        int[] getallen = new int[7] { 0, 1, 2, 3, 4, 5, 6, 7 };

        // 2. Ontwerpen van de query
        // let op!!
        // deze query variabele is niet van het type int(eger)!
        var query =
            from num in numbers
            where (num % 2) == 0
            select num;

        // 3. Uitvoeren van de query
        foreach (int num in numQuery)
        {
            Console.Write("{0,1} ", num);
        }
    }
}
```
Kijkt men daar bovenstaande code vind men dat de query parameter geen integer is, in feite is hij van het type IEnumerable<int>, de variabele query is dus een enumerable collection die integers houdt. Om de integers van de collection uit te printen wordt de *LINQ-query uitgevoerd*.

**LINQ-querys zijn enkel toepasbaar op 'dingen', die het type IEnumerable ondersteunen, vandaar de LINQ-query ontwerp.**

# LINQ met XML
Stel men heeft volgende datasource waar men wenst een LINQ query mee uit te voeren. Deze datasource is een verkoopsgeschiedenis waar in het:
- Aankomstadres
- Facuuradres
- Afleveringsnotitie voor de pakjesdienst
- De verstuurde goederen
    - Onderdeelnummer
    - Meer boeiende info over het onderdeel.
```XML
<!-- PurchaseOrder.xml -->
<?xml version="1.0"?>  
<PurchaseOrder PurchaseOrderNumber="99503" OrderDate="1999-10-20">  
  <Address Type="Shipping">  
    <Name>Ellen Adams</Name>  
    <Street>123 Maple Street</Street>  
    <City>Mill Valley</City>  
    <State>CA</State>  
    <Zip>10999</Zip>  
    <Country>USA</Country>  
  </Address>  
  <Address Type="Billing">  
    <Name>Tai Yee</Name>  
    <Street>8 Oak Avenue</Street>  
    <City>Old Town</City>  
    <State>PA</State>  
    <Zip>95819</Zip>  
    <Country>USA</Country>  
  </Address>  
  <DeliveryNotes>Please leave packages in shed by driveway.</DeliveryNotes>  
  <Items>  
    <Item PartNumber="872-AA">  
      <ProductName>Lawnmower</ProductName>  
      <Quantity>1</Quantity>  
      <USPrice>148.95</USPrice>  
      <Comment>Confirm this is electric</Comment>  
    </Item>  
    <Item PartNumber="926-AA">  
      <ProductName>Baby Monitor</ProductName>  
      <Quantity>2</Quantity>  
      <USPrice>39.98</USPrice>  
      <ShipDate>1999-05-21</ShipDate>  
    </Item>  
  </Items>  
</PurchaseOrder>
```
**Vraag:** Stel we willen van deze source alle onderdeelnummers te weten komen, we kunnen dit als volgt oplossen door middel van LINQ.
1) Laad de datasource-file in als een variabele>
```C#
XElement order = XElement.Load(
    Path.Combine(Directory.GetCurrentDirectory(), "PurchaseOrder.xml"));
```
2) Ontwerp de LINQ query.
In woorden:
>   Voor (een/elk) item in de order in het Item-veld geef mij het Onderdeelnummer.
3) Voer de LINQ query uit.
Dus dit in code met in ons achterhoofd dat een LINQ query een IEnumerable<T> terug geeft.
```C#
IEnumerable<string> partNos =  from item in order.Descendants("Item")
                               select (string) item.Attribute("PartNumber");
foreach (var item in partNos)
    Console.WriteLine(item);
```

### Iets meer diepgaand voorbeeld
**Vraag:** Zelfde vraag alleen voor de item(s) waarvan de USPrice groter is dan 100.
```C#
var partNum = from item in order.Descendants("Item")
              where (double)item.Element("USPrice") > 100
              select (string)item.Attribute("PartNumber");
```
Uitleg van de code:
- `from item in order.Descendants("Item")`: selecteer de Item-velden
- `where (double)item.Element("USPrice") > 100` van zulk vorig geselecteerd Item-veld kijk of het USPrice element in het item groter is dan 100.
- `select (string)item.Attribute("PartNumber");` geef de partnumber collection terug

### Volledige code:
(Voor een .NET Framework console application)
```C#
static void Main(string[] args)
{
    XElement order = XElement.Load(
        Path.Combine(Directory.GetCurrentDirectory(), "PurchaseOrder.xml"));
    var partNum = from item in order.Descendants("Item")
                  where (double)item.Element("USPrice") > 100
                  select (string)item.Attribute("PartNumber");

    foreach (var item in partNum)
        Console.WriteLine(item);
    Console.ReadLine();
}
```
## Primer: Method Syntax
De method syntax schrijfwijze is een alternatief voor de meer traditionele query syntax (de from ... select). Het laat toe voor een meer vloeiende schrijfwijze aangezien de meeste mensen meer geneigd zijn om methodes te schrijven dan query-syntax. Hier een voorbeeld waarin het verschil duidelijk wordt.
 ```C#
var numbers = Enumerable.Range(1, 100); //1,2,...,100
//query syntax:
var query = from n in numbers
    where n % 3 == 0
    select n * 2; // 3 en zijn veelvouden verdubbeld terug geven.
//method syntax:
var method = numbers
    .Where(n => n % 3 == 0)
    .Select(n => n * 2);
```
# TODOs
## TODO Functional Construction
Het maken van XML Tree's adhv 1 statement [Functional Construction](https://docs.microsoft.com/en-us/dotnet/csharp/programming-guide/concepts/linq/functional-construction-linq-to-xml).