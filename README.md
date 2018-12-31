# andromeda

Quickly create large amounts of unique serial numbers.  Heavily
inspired by [random code generator](https://www.randomcodegenerator.com/en/generate-serial-numbers).

## Creating Unique Serial Numbers

Making a batch of unique licenses is very simple. Run the application with:

```shell
./andromeda
```

Let's go ahead and create 1000 serial numbers.  We want the serial numbers to be
20 symbols long and we want to include numbers, uppercase letters, and lowercase
letters:

```text
Serial number amount:
1000
Serial number length (20 or less):
20
Enter 'y' to use numbers:
y
Enter 'y' to use uppercase letters:
y
Enter 'y' to use lowercase letters:
y
```

When we are done, we will find a new file within the same folder as this
application named `1000_unique_serials.txt`.  Opening the file, we see:

```text
dPzu8znblSPF1rmy4coW
djKD1ZxNLqAAo6ah6ZuA
dCZbvdHO9KIvm3CAnNCE
dzfN89poHQ7QMxZgGKH1
dqnj4G4A3mh23SpeoaZG
drsWemJsfg6aCzw0wHge
dYGS8jevEv5T0PFnd1bZ
dDSR4ZrQ2JgVZT2GmV4t
doO2eKULC3JLfuWBhQys
dp5yr9w1PrPpuAIWcGMH
dgFnW1Eppccc5gRNEFAR
dWosgmKFvdFtbKvCDB5X
dyTmrjnIhLK1pwhmXr9V
dMJQWVxqxRSjyt5lIOey
dwbPgKHTq5frkOzHJ0wP
dFheZibfSFeXVlPfzeSg
...
```

## How it Works

1. Check to make sure that it is possible to create the amount of unique
   licenses requested based on the the serial number length and character set.  
   For example, if the user requests 1,000,000 unique licenses, but only asks
   for serial numbers of length 4 and wants only numbers as symbols, it will not
   be possible, as 10^4 < 1,000,000.  We must have more possible combinations
   than the number of licenses requested in order to ensure all licenses will be
   unique.
2. Make a `character_vector` that is comprised of all the symbols the user
   wishes to use in their serial numbers (numbers, lowercase letters, uppercase
   letters).
3. Shuffle this vector and push a copy of it into another vector, called
   `vector_of_character_vectors`.  This step is executed as many times as the
   length of the serial number requested.
4. Create a new vector of numbers called `index_vector`.  Each cell in this
   vector holds an index that corresponds to a `character_vector` in the
   `vector_of_character_vectors`.
5. A serial number is generated by using the `index_vector` to provide an index
   to each of the corresponding internal character vectors in
   `vector_of_character_vectors`.  The `vector_of_character_vectors` contains
   as many internal `character_vector`s as the length of our serial number.  
   This can be seen sort of as a speedometer that starts with 0000 miles.  As we
   roll through the miles in the rightmost column, it resets back to zero and
   the next value to the left increments by 1.  This is exactly how the
   `index_vector` and `vector_of_character_vectors` work to obtain serial
   numbers.

In order to avoid printing serial numbers that look like this:

```text
PWZdySrxntWyBxYYZAlT
PWZdySrxntWyBxYYZAlB
PWZdySrxntWyBxYYZAlZ
PWZdySrxntWyBxYYZAlJ
...
```

we need to pull serial numbers evenly over the entire spectrum of possible
combinations.  We can do this by taking the total possible combinations based
on the user settings and dividing it by the number of licenses needed.  This
will give us the value that we need to be apply to the `index_vector` before
printing the next serial number to the file.  Using the example from earlier:

```text
Serial number amount:
1000
Serial number length (20 or less):
20
Enter 'y' to use numbers:
y
Enter 'y' to use uppercase letters:
y
Enter 'y' to use lowercase letters:
y
```

Our licenses are of length 20 and use 62 types of symbols (10 numbers + 26
lowercase letters + 26 uppercase letters).  We have 62^20 possible unique
serial number combinations.  Because we want 1000 licenses, the spacing between
serial numbers to be printed to the file is (62^20)/1000 =
704423425546998022968330264616370.  If we set
n = 704423425546998022968330264616370, then the function
`increase_index_vector_by()` efficiently applies this number to the
`index_vector` in roughly (log(n) with base 62) operations.  Using this method, we
are able to print licenses that are equally spaced apart in the total
combination space, as shown in the first sample output earlier.

We can even view the output of the `index_vector` to see the patterns better by
uncommenting the function `print_index_vector()`:

```text
00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
00 03 52 20 20 51 36 12 55 34 13 55 03 29 16 22 50 36 43 40
00 07 42 40 41 41 10 25 49 06 27 48 06 58 32 45 39 11 25 18
00 11 32 61 00 30 46 38 42 40 41 41 10 25 49 06 27 48 06 58
00 15 23 19 21 20 20 51 36 12 55 34 13 55 03 29 16 22 50 36
00 19 13 39 42 09 57 02 29 47 07 27 17 22 19 52 04 59 32 14
00 23 03 60 00 61 31 15 23 19 21 20 20 51 36 12 55 34 13 54
00 26 56 18 21 51 05 28 16 53 35 13 24 18 52 35 44 08 57 32
00 30 46 38 42 40 41 41 10 25 49 06 27 48 06 58 32 45 39 10
00 34 36 59 01 30 15 54 03 60 00 61 31 15 23 19 21 20 20 50
00 38 27 17 22 19 52 04 59 32 14 54 34 44 39 42 09 57 02 28
00 42 17 37 43 09 26 17 53 04 28 47 38 11 56 02 60 31 46 06
00 46 07 58 01 61 00 30 46 38 42 40 41 41 10 25 49 06 27 46
00 49 60 16 22 50 36 43 40 10 56 33 45 08 26 48 37 43 09 24
00 53 50 36 43 40 10 56 33 45 08 26 48 37 43 09 26 17 53 02
00 57 40 57 02 29 47 07 27 17 22 19 52 04 59 32 14 54 34 42
...
```

## Pitfalls to be Aware of

Andromeda does not care if you choose settings that result in a very low pool of
license combinations.  You should be aware of this.  If you run the application
with the following options:

```text
Serial number amount:
1000
Serial number length (20 or less):
4
Enter 'y' to use numbers:
y
Enter 'y' to use uppercase letters:
n
Enter 'y' to use lowercase letters:
n
```

the output will be:

```text
9444
9474
9494
9464
9484
9434
9424
9404
9414
9454
9244
9274
9294
9264
9284
9234
...
```

Notice that the licenses are fairly similar.  Also, note that it would be fairly
easy to guess a serial number.  The probability that a random guess would be an
actual serial number is 1000/(10^4) = 0.1.  It is up to the user to understand
this and adjust the settings to increase the complexity of the output and
decrease the chances of guessing a license number.  Using the example from
earlier with 1000 serial numbers of length 20 using all symbols, the probability
that a random guess would be an actual serial number is 1000/(62^20) =
1.4196007e-33.
