
//Variables 
/*
            Variables numéricas en Rust
            En Rust, encontrarás diferentes tipos de variables numéricas para el guardado de números con mayor o menos longitud y con/sin signo.

            Length	Signed	Unsigned
            8-bit	 i8	       u8
            16-bit	 i16	   u16
            32-bit	 i32       u32
            64-bit	 i64	   u64
            128-bit	 i128	   u128

            Por ejemplo, i8 permite guardar números hasta 8 bits. 50% para números positivos y el otro 50% para números negativos. Mientras que u8, al no tener signo, permite guardar solo números positivos. Esto permite tener más capacidad de manipular números y realizar operaciones.

            DataType  Min	                    Max	                            Length
            i8	     -128	                    127	                             8-bit
            i16	    -32768	                    32767	                        16-bit
            i32	    -2147483648 	            2147483647	                    32-bit
            i64	    -92233720368547… 	        92233720368547…	                64-bit
            i128    -17014118346046923173168…	17014118346046923173168…    	128-bit

            Si tienes la necesidad de guardar números extremadamente grandes, puedes utilizar u128 para guardar el doble de números positivos en comparación con lo que permite i128.

            DataType   Min	Max
            u8	        0	255
            u16	        0	65535
            u32	        0	4294967295
            u64	        0	18446744073909551615
            u128	    0	340282366920938463463374607431768211455
            
            Declaración de variables numéricas en Rust
            Rust es un lenguaje fuertemente tipado, lo que significa que tienes que hacer explícito el tipo de las variables y este no pueden cambiar en tiempo de ejecución.
            Para declarar una variable y su tipo, hazlo haciendo uso de la palabra reservada let y el tipo de la misma:

            fn main() {
                let a: i16 = 100;
                let b: u16 = 200;
                println!("El número es {} y {}", a, b);     // El número es 100 y 200
            }
            Para utilizar y mostrar por pantalla dichas variables, utiliza el comando println!() e ingresa el texto que deseas mostrar. Las variables que le pases después de la primera coma, reemplazarán los {}, denominados “placeholders”, y formateará el texto en el orden que hayas establecido el ingreso de cada valor.

            Por último, si deseas permitir que el valor de una variable sea modificable, tienes que agregar explícitamente la palabra reservada mut en la declaración de la misma, ya que, por defecto, no puedes modificar el valor de una variable.

            fn main() {
                let mut a: i16 = 100;
                let b: u16 = 200;
                a = a + 1;
                println!("El número es {} y {}", a, b);     // El número es 101 y 200
            }
            Variables del tipo string en Rust
            Para el guardado de variables del tipo cadenas de texto, el tipado se realiza con la palabra reservada &str.

            fn main() {
                let name: &str = "Kevin";
                println!("Mi nombre es {}", name);      // Mi nombre es Kevin
            }
            Recuerda que puedes correr tu proyecto de Rust con el comando cargo run y visualizar los mensajes y los valores de las variables desde la terminal de línea de comandos.

            De esta manera, ya sabes los conceptos básicos sobre declaración de variables y tipado de las mismas en Rust, además de mostrar sus valores por pantalla.

*/

//Inputs
/*
            Permitir la interacción del usuario con tu aplicación es una de las bases de cualquier lenguaje de programación. Rust facilita la comunicación a través de la interfaz de línea de comandos. Veamos cómo lograrlo:

            Inputs en Rust
            Para recibir datos del usuario, tienes que hacer uso de algunas librerías internas de Rust que acceden al sistema operativo y permiten que el usuario ingrese datos por consola.

            fn main() {
                println!("Ingrese su nombre:");
                let mut nombre: String = String::new();
                std::io::stdin().read_line(&mut nombre).unwrap();
                nombre = nombre.trim().to_string();

                println!("Bienvenido o bienvenida: {}", nombre);
            }
            
            Si tienes experiencia en lenguajes como C++ o Java, tal vez te sea más familiar algunos conceptos o palabras reservadas. Veamos qué es cada línea de código y cómo funciona:

            println!("Ingrese su nombre:");   // Un simple mensaje por consola para solicitarle el nombre al usuario.
            let mut nombre: String = String::new();   // Declaramos una variable del tipo string donde almacenaremos el nombre. Nota que puedes declarar un string con String o con &str. Las diferencias son mínimas, el primero te permite manipular el texto, hacer substrings o splits, pero ocupa algo más de espacio en memoria. El segundo es texto plano sin otra funcionalidad.
            std::io::stdin().read_line(&mut nombre).unwrap();  // La línea más complicada, std es una librería de Rust para acceder al sistema operativo. io significa inputs/outputs, println!() lo utiliza internamente para mostrar mensajes por consola, aquí lo utilizaremos para ingresar datos por la misma. stdin() permite traer información y read_line() indica que esa información será recibida por consola. &mut nombre es la variable donde guardaremos el dato ingresado por el usuario (Utiliza mut en las variables para indicar que la misma cambiará de valor). Finalmente, unwrap(), nos ayuda con el manejo de errores.
            nombre = nombre.trim().to_string();   //Aquí solo formateamos el texto para eliminar saltos de línea y espacios en blanco.
            println!("Bienvenido o bienvenida: {}", nombre);  // Luego de que el usuario ingresara su nombre, lo mostramos por consola.
            
            Recuerda que puedes ejecutar el código con el comando cargo run.

            Cambiando el tipo de dato en Rust
            Por defecto, todos los datos que ingresa el usuario por consola son del tipo string. Puede ocurrir que necesites números enteros y para esto tienes que convertir el tipo de datos de la siguiente manera:

            fn main() {
                println!("Ingrese su edad:");
                let mut edad: String = String::new();
                std::io::stdin().read_line(&mut edad).unwrap();
                let edad_int: u8 = edad.trim().parse().unwrap();

                println!("La edad es: {}", edad_int);
            }

            Lo importante aquí es la línea let edad_int: u8 = edad.trim().parse().unwrap(); donde creamos una nueva variable donde guardaremos un número del tipo u8. De esta forma podrás manipular ese dato y realizar cualquier operación matemática.
            Hasta aquí, ya sabes declarar varios tipos de variables, recibir y mostrar datos por consola. Juega con estas herramientas creando un formulario para que el usuario ingrese valores por consola y pueda manipularlos antes de mostrarlos por la terminal.


*/
//Condicionales 

/*
            Rust, como cualquier lenguaje Turing Completo, permite desarrollar programas con lógica y condicionantes para realizar una u otra acción dependiendo el valor de las variables o el estado de ejecución del programa.

            If en Rust
            Un if es un if, en C++, en Javascript, en Ruby o cualquier otro lenguaje. La sintaxis de un if/else en Rust es algo particular:

            fn main() {
                let edad: i8 = 20;
                if edad >= 18 {
                    println!("Eres mayor de edad");
                } else {
                    println!("Eres menor de edad");
                }
            }
            En Rust no utilizamos los paréntesis para declarar la condición del if. Puedes hacerlo, tu código funcionará, pero recibirás algún warning por consola diciéndote que los remuevas.

            La condición de un if puede ser de varios tipos, desde validar la igualdad de datos con == o verificar si un número es mayor, menor o igual con >, <, >= y <=. También puedes agrupar condiciones con && para un AND lógico o un || para un OR lógico.

            fn main() {
                let edad: i8 = 20;
                if edad >= 18 && edad <= 70 {
                    println!("Eres mayor de edad y tienes menos de 70 años");
                }
            }
            Puedes realizar las típicas operaciones de validación y comparaciones de datos con un if. De esta forma, ramificar tu código en distintas operaciones a partir de una condición y cambiar su comportamiento.

*/

//Ciclos loops
/*
            Los ciclos iterativos en Rust realmente rompen con el paradigma de sintaxis de programación en comparación con otros lenguajes.

            Iteraciones en Rust
            Si tienes algo de experiencia en PHP, Java, Javascript, C#, C++, o cualquier otro lenguaje, entenderás un ciclo iterativo como una repetición de un código N cantidad de veces. Rust no es la excepción, puedes declarar una iteración de una forma particular:

            fn main() {
                loop {
                    println!("Ingrese 123 para detener el loop:");
                    let mut number: String = String::new();
                    std::io::stdin().read_line(&mut number).unwrap();
                    let number_int: i32 = number.trim().parse().unwrap();

                    if number_int == 123 {
                        break;
                    }
                }
            }
            La palabra reservada loop crear un ciclo de iteraciones del código fuente en su interior. El ciclo se repetirá hasta encontrar un break.
            En el anterior ejemplo, el ciclo se repite infinitas veces hasta que el usuario ingrese el número 123. Validamos el número ingresado con un if y, si este es correcto, finalizamos la ejecución del loop.

            loop es una manera diferente de crear un ciclo iterativo. Es crucial que tengas los cuidados necesarios en tu algoritmo y que sea bien probado para no entrar en bucles infinitos y asegurar que el mismo tenga un punto de finalización.

*/

fn main() {
    fn variables(){
            //Declare variables en rush
            let a: i16 = 100;
            let b: i16 = 200;
            //Suma
            let sum: i16 = a + b;

            //Nombre
            let nombre :&str = "perra";
            let apellido :&str = "martinez";

            //Temperaturas
            let temp_colomba :i16 = 29;
            let temp_canada :i16 = -10;

            //Imprime
            println!("Suma :{}" ,sum);
            println!("Nombre y Apellido : {} {}",nombre,apellido);
            println!("Temperatura colombia es :{}  Temperatura canada es :{}",temp_colomba,temp_canada); // println! con salto de linea  //print sin salto de linea
    }

    fn inputs(){
            //Titulo
            println!("Registra una palabra");
            //Declare 
            //let titulo: String  = "Registre una palabra :";
            let mut palabra_input: String = String::new(); // String vacio

            //Pedir palabra por consola.
            std::io::stdin().read_line(&mut palabra_input).unwrap();

            //Imprimir en consola
            println!("palabra registrada : {}",palabra_input);
    }

    fn get_persona_datos(){
            //Declare variables
            let mut nombre:String = String::new();
            let mut apellido:String = String::new();
            let mut edad:String = String::new();
            let mut pais:String = String::new();     

            //Pedir nombre completo y edad por consola
            //Titulo
            //to_string() 
            println!("Ingrese nombre :"); 
            nombre = nombre.trim().to_string();       
            std::io::stdin().read_line(&mut nombre).unwrap();

            println!("Ingrese apellido :");
            apellido = apellido.trim().to_string(); 
            std::io::stdin().read_line(&mut apellido).unwrap();

            println!("Ingrese pais :");
            pais = pais.trim().to_string(); 
            std::io::stdin().read_line(&mut pais).unwrap();

            println!("Ingrese edad :");
            std::io::stdin().read_line(&mut edad).unwrap();
            
             //Edad a int /String -> int
            //trim() elimina espacio del inicio y final
            //parse() convierte a entero (int)
            //unrap() ayuda a controlar y manejar errores
            let edad_int:i8 = edad.trim().parse().unwrap();

            //Impriimir
            println!("Nombre : {}  Apellido : {} Con edad : {} Del pais : {}",nombre,apellido,edad_int,pais);

    }

    fn condictionales_example(number: i16){
    let titulo:&str = "Club de Putas";
    let reglas:&str = "-Ser mayor de edad 18";
    println!("Bienvenido a {} Reglas :{}",titulo,reglas);

        if number >= 18{
            println!("Eres un adulto, puedes entrar. Edad: {}",number);
            if number >= 30{
                println!("Eres adulto, pero no toleras ya bien el alcohol y las putas");
                if number >= 50{
                    println!("Eres adulto pero tienes condicion especial al entrar nada de mujeres de 18.");
                    if number > 60 && number < 55{
                        println!("Eres adulto pero no puedes entrar, ya no se te para.");
                    }
                }
            }
        }else{
            println!("Prohibido el ingreso a los menores de edad y personas que no cumplan el estandar de calidad");
        }
    }

    fn condictionales_example2(number:i8){
        let titulo:&str = "Club de Programadores";
        let reglas:&str = "-Ser mayor de edad 18";
        println!("Bienvenido a {} Reglas :{}",titulo,reglas);

        if number >= 18{
            println!("Eres un adulto, puedes entrar al club. Edad: {}",number);
        }else if number == 30 {
            println!("Eres un adulto, pero debes ser senior-fullstack. Edad: {}",number);
        }else if number == 40 {
            println!("Eres un adulto, pero debes ser ingeniero. Edad: {}",number);
        }else{
            println!("No eres adulto sigue estudiando o obten una acreditacion. Edad: {}",number); 
        }
    }

    /*
    Funcion escoger la pildora roja o azul de matriz,  si escoge roja se unen a la resistencia y si escoge azul explota matrix
    debe ser con strings y que esten en minuscula
    si no escoge o escribe las palabras asignadas debe determinar que no tomo una decision
    */
    fn escoger_a_r_matrix(color:&str){
        let mut pill: String = String::new();
        //std::io::stdin().read_line(&mut pill).unwrap();

        let titulo:&str = "BIENVENIDO A MATRIX";
        let reglas:&str = "Toma una decision entre ROJO Y AZUL";
        let rojo_s:&str = "Te uniste a la resistencia en matrix y vez el final hasta el conejo";
        let azul_s:&str = "Vuelves a la matrix y no paso nada";

        let rojo:&str = "rojo";
        let azul:&str = "azul";

        pill = color.trim().to_lowercase();

        println!("{}",titulo);
        println!("{}",reglas);

        if pill == rojo{
            println!("{} color: {}",rojo_s,pill);
        }else if pill == azul{
            println!("{} color: {}",azul_s,pill);
        }else{
            println!("Explota la matrix broo o escribiste mal tu palabra.");
        }

    }

    fn sumar(number1:i128,number2:i128){
        let titulo:&str = "Bienvenido podemos sumar lo que sea:";
        let suma:i128 =  number1 + number2;
        loop {
            println!("FUNCION SUMAR - {}", titulo);
            println!("SUMANDO escribe tu suma:  n1 :{} y n2 :{}", number1,number2);
            let mut suma_persona:String = String::new();
            //Pedir al usuario la suma de el
            std::io::stdin().read_line(&mut suma_persona).unwrap();
            let suma_int_persona :i128 =  suma_persona.trim().parse().unwrap();
            
            if suma_int_persona == suma {
                println!("La suma es correcta bro : {}",suma);
                break;
            }else{
                println!("La suma no es correcta bro : {}",suma_int_persona);
            }
        }
    }





    //Iniciar funciones
    //variables();
    //inputs();
    //get_persona_datos();
    //condictionales_example(18);
    //condictionales_example2(20);
    //escoger_a_r_matrix("azul");
    sumar(22, 22);

}
